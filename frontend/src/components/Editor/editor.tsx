import Editor, { Monaco, useMonaco } from "@monaco-editor/react";
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
import { useEffect, useRef, useState } from "react";
import EightOEightFiveDefinition from "../../monaco/languageDefinition";
import { Range } from 'monaco-editor/esm/vs/editor/editor.api';


type CodeEditor = monaco.editor.IStandaloneCodeEditor;

function CodeEditor({ dec, setDec ,changeHandler }: { dec: string[] , setDec: (l: string[]) => void, changeHandler: () => void ,}) {
    const monaco = useMonaco();
    let [code, setCode] = useState<string[]>([]);
    let editorRef = useRef<CodeEditor | null >();

    function handleEditorDidMount(editor: monaco.editor.IStandaloneCodeEditor, _: Monaco) {
        console.log("editor mounted");
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
        //editorRef.current?.dispose();
        stopDecoration();
        var item = localStorage.getItem("lineToHighlight");
        if (item != null) {
            editorRef.current?.setPosition({ column: 1, lineNumber: parseInt(item) });
            let decs = editorRef.current?.deltaDecorations([], [{
                range: new Range(parseInt(item), 1, parseInt(item), 1),
                options: {
                    isWholeLine: true,
                    className: 'runDecorator'
                }
            }]) as string[];
            localStorage.setItem("decoration", decs[0]);
            setDec(decs);
            console.log(decs);
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
        editorRef.current?.deltaDecorations(["b;11"], []);
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
