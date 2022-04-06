import React from 'react';
import logo from './logo.svg';
import { useEffect } from 'react';

import Editor, {useMonaco} from "@monaco-editor/react";
import Flags from './components/Flags/flags';
import Registers from './components/Register/registers';
import EightOEightFiveDefinition from './monaco/languageDefinition';
import MemoryView from './components/MemoryView/memory_view';

function App() {
  const monaco = useMonaco();
  useEffect(() => {
      if (monaco) {
        monaco.languages.register({id: '8085asm'});
        monaco.languages.setMonarchTokensProvider('8085asm', EightOEightFiveDefinition());
      }
  }, [monaco]);
  

  return (
    <div className="mainWindow">
      <Editor
        height="90vh"
        width="75vw"
        defaultLanguage="8085asm"
        defaultValue="; type here"
        options = {{fontSize : 20,
          minimap: {enabled: false}
        }}
        theme ="vs-dark"
      />
      <div className="states">
        <Flags/>
        <Registers/>
        <MemoryView/>
      </div>
    </div>
  );
}

export default App;
