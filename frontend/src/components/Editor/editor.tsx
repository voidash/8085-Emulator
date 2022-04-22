import Editor, { Monaco, useMonaco } from "@monaco-editor/react";
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
import { useEffect, useRef } from "react";
import EightOEightFiveDefinition from "../../monaco/languageDefinition";

type CodeEditor = monaco.editor.IStandaloneCodeEditor;

function CodeEditor({ onChange }: { onChange: () => void }) {
    const editorRef = useRef<CodeEditor | null>(null);
    const monaco = useMonaco();

    function handleEditorDidMount(editor: monaco.editor.IStandaloneCodeEditor, _: Monaco) {
        editorRef.current = editor;
    }

    useEffect(() => {
        if (monaco) {
            monaco.languages.register({ id: '8085asm' });
            monaco.languages.setMonarchTokensProvider('8085asm', EightOEightFiveDefinition());
        }
    }, [monaco]);

    return (
        <Editor
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