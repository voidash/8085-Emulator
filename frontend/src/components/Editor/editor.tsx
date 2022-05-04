import Editor, { Monaco, useMonaco } from "@monaco-editor/react";
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
import { useEffect, useRef, useState } from "react";
import EightOEightFiveDefinition from "../../monaco/languageDefinition";
import { Range } from 'monaco-editor/esm/vs/editor/editor.api';


type CodeEditor = monaco.editor.IStandaloneCodeEditor;

function CodeEditor({ changeHandler }: { changeHandler: () => void }) {
    const editorRef = useRef<CodeEditor | null>(null);
    const monaco = useMonaco();
    let [decoration, setDecoration] = useState<string[] | undefined>();

    let [code, setCode] = useState<string[]>([]);

    function handleEditorDidMount(editor: monaco.editor.IStandaloneCodeEditor, _: Monaco) {
        editorRef.current = editor;
    }

    var onChange = () => {
        let codeBuffer: String = editorRef.current?.getValue() as String;
        setCode(codeBuffer.split("\n"));
        localStorage.setItem("code", codeBuffer.toString());
        changeHandler();
        stopDecoration();
    }

    useEffect(() => {
        if (monaco) {
            monaco.languages.register({ id: '8085asm' });
            monaco.languages.setMonarchTokensProvider('8085asm', EightOEightFiveDefinition());
        }
        window.addEventListener('highlightChange', () => {
            gotoLine();
        });

        window.addEventListener('8085-warning', () => {
            showWarning();
        });

        window.addEventListener('8085-error', () => {
            showError();
        });
    }, [monaco]);





    function gotoLine() {
        var item = localStorage.getItem("lineToHighlight");
        if (item != null) {
            editorRef.current?.setPosition({ column: 1, lineNumber: parseInt(item) });
            setDecoration(editorRef.current?.deltaDecorations([], [{
                range: new Range(parseInt(item), 1, parseInt(item), 40),
                options: {
                    isWholeLine: true,
                    className: 'runDecorator'
                }
            }]));
        } else {
            stopDecoration();
        }
    }


    function showWarning() {
        var item = localStorage.getItem("8085-warning");
        if (item) {
            var error = JSON.parse(item);
            monaco?.editor.setModelMarkers(editorRef.current?.getModel() as monaco.editor.ITextModel, '8085-warning', [{
                startLineNumber: error["line"],
                startColumn: 1,
                endLineNumber: 2,
                endColumn: 1000,
                message: error["message"],
                severity: monaco.MarkerSeverity.Warning
            }]);
        } else {
            stopDecoration();
        }
    }

    function showError() {
        var item = localStorage.getItem("8085-error");
        if (item) {
            var error = JSON.parse(item);

            monaco?.editor.setModelMarkers(editorRef.current?.getModel() as monaco.editor.ITextModel, '8085-error', [{
                startLineNumber: error["line"],
                startColumn: 1,
                endLineNumber: 2,
                endColumn: 1000,
                message: error["message"],
                severity: monaco.MarkerSeverity.Error
            }]);
        } else {
            stopDecoration();
        }
    }


    function stopDecoration() {
        if (decoration) {
            editorRef.current?.deltaDecorations(decoration as string[], []);
        }
        monaco?.editor.setModelMarkers(editorRef.current?.getModel() as monaco.editor.ITextModel, '8085-warning', [])
        monaco?.editor.setModelMarkers(editorRef.current?.getModel() as monaco.editor.ITextModel, '8085-error', [])
    }


    return (
        <Editor
            value={localStorage.getItem("code") ?? "; Type Your Code Here"}
            height="70vh"
            width="100%"
            onChange={onChange}
            defaultLanguage="8085asm"
            defaultValue="; Type your code here"
            options={{
                fontSize: 20,
                minimap: { enabled: false }
            }}
            theme="vs-dark"
            onMount={handleEditorDidMount}
        />
    );
}

export default CodeEditor;