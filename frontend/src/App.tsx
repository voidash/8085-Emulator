import {useRef,useState} from 'react';
import { useEffect } from 'react';
// monaco stuff
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
import {Range} from 'monaco-editor/esm/vs/editor/editor.api';
import Editor, { Monaco, useMonaco} from "@monaco-editor/react";

import Flags from './components/Flags/flags';
import Registers from './components/Register/registers';
import EightOEightFiveDefinition from './monaco/languageDefinition';
import MemoryView from './components/MemoryView/memory_view';
//wasm stuff
import * as wasm from './wasm/wasm_8085';

type CodeEditor = monaco.editor.IStandaloneCodeEditor;

function App() {
  // equivalent to programCounter 
  let [line, setLine] = useState<number>(0);
  let [code, setCode]  = useState<string[]>([]);
  let [loaded, setLoaded]  = useState<boolean>(false);
  let [decoration, setDecoration] = useState<string[] | undefined>();
  let [emulator, setEmulator] = useState<wasm.Emulator | null>(null);
  const editorRef= useRef<CodeEditor|null>(null);
  const monaco = useMonaco();

  //load emulator
  
  function loadEmulator(callback: () => void) {
      wasm.default().then(() => {
        callback();
     });
  }
  useEffect(() => {
    loadEmulator(() => {
     if (emulator == null) {
        setEmulator(new wasm.Emulator(0));
      };
    });
  },[]);

  // load monaco
  useEffect(() => {
      if (monaco) {
        monaco.languages.register({id: '8085asm'});
        monaco.languages.setMonarchTokensProvider('8085asm', EightOEightFiveDefinition());
      }
  }, [monaco]);
  
  function handleEditorDidMount(editor: monaco.editor.IStandaloneCodeEditor, _: Monaco){
    editorRef.current = editor;
  }

  function showValue() {
    setLoaded(false);
    let codeBuffer: String = editorRef.current?.getValue() as String;
    setCode(codeBuffer.split("\n"));
  }

  function gotoLine(linenumber: number) {
    editorRef.current?.setPosition({column: 1, lineNumber: linenumber});
    setDecoration(editorRef.current?.deltaDecorations([], [{
      range: new Range(linenumber,1,linenumber,40),
      options: {
        isWholeLine: true,
        className: 'runDecorator'
      }
    }]));
  }
  function stopDecoration() {
    editorRef.current?.deltaDecorations(decoration as string[], []);
  }

  function assemble() {
     
  }

  function debugMode() {
    //set program counter 
    setLine(emulator?.program_counter() as number);
    console.log(line);
    if(loaded) {
      emulator?.emulate_line_by_line();
      setLine(emulator?.program_counter() as number);
      gotoLine(line);
    }else {
      console.log("Program not loaded");
    }
    console.log(emulator?.program_counter());
    stopDecoration();
  }

  function loadProgram() {

    //force update
    loadEmulator(() => {
      setEmulator(new wasm.Emulator(0));
      emulator?.load_program(code);
      emulator?.set_program_counter(0);
      setLoaded(true);
    });
    stopDecoration();
  }


  return (
    <div className="mainWindow">
      <Editor
        height="90vh"
        width="65vw"
        onChange={showValue}
        defaultLanguage="8085asm"
        defaultValue="; Type your code here"
        options = {{fontSize : 20,
          minimap: {enabled: false}
        }}
        theme ="vs-dark"
        onMount = {handleEditorDidMount}
      />
      <div className="states">
      <button onClick={() => assemble()}> Assemble</button>
      <button onClick={() => loadProgram()}> Load Program</button>
      <button onClick={() => debugMode()}> Debug Mode</button>
      <button onClick={() => stopDecoration()}> Run Mode</button>
        <Flags/>
        <Registers/>
        {emulator == null ? "loading" : <MemoryView emulator={emulator as wasm.Emulator} loaded={loaded}/>};
      </div>
    </div>
  );
}

export default App;
