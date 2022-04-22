import { useRef, useState } from 'react';
import { useEffect } from 'react';
import { Range } from 'monaco-editor/esm/vs/editor/editor.api';

import Flags from './components/Flags/flags';
import Registers from './components/Register/registers';
import MemoryView from './components/MemoryView/memory_view';
//wasm stuff
import * as wasm from './wasm/wasm_8085';

import { Log, LogType } from './utils/log';
import { Box, Button, Collapse, Stack, Tab, Tabs, } from '@mui/material';
import { BugReport, Build, RunCircle, SimCardDownload } from '@mui/icons-material';
import { TabPanel } from './components/TabPanel/tab_panel';
import { useSnackbar } from 'notistack';
import CodeEditor from './components/Editor/editor';

function App() {

  const [tabvalue, setTab] = useState(0);

  const handleChange = (event: React.SyntheticEvent, newValue: number) => {
    setTab(newValue);
  };


  let [line, setLine] = useState<number>(1);
  let [log, setLog] = useState<Log>({ type: LogType.NOTHING, LogString: "" })
  let [pcLineVec, setPcLineVec] = useState<Uint32Array>(new Uint32Array());
  let [code, setCode] = useState<string[]>([]);
  let [loaded, setLoaded] = useState<boolean>(false);
  let [decoration, setDecoration] = useState<string[] | undefined>();
  let [emulator, setEmulator] = useState<wasm.Emulator | null>(null);
  const editorRef = useRef<CodeEditor | null>(null);
  const { enqueueSnackbar } = useSnackbar();

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
  }, []);


  var showValue = () => {
    setLoaded(false);
    let codeBuffer: String = editorRef.current?.getValue() as String;
    setCode(codeBuffer.split("\n"));
  }

  function gotoLine(linenumber: number) {
    editorRef.current?.setPosition({ column: 1, lineNumber: linenumber });
    setDecoration(editorRef.current?.deltaDecorations([], [{
      range: new Range(linenumber, 1, linenumber, 40),
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
    if (loaded) {

      emulator?.emulate_line_by_line();
      let val = pcLineVec.findIndex((val) => {
        return emulator?.program_counter() as number === val;
      });

      console.log("val is " + val);
      setLine(val + 1);
      gotoLine(val + 1);
      stopDecoration();
    }
  }

  function loadProgram() {
    //check if program contains hlt at the end of the code otherwise issue warning 
    // if(!code[-1].toLowerCase().trim().startsWith("hlt")) {
    // setLog({type: LogType.WARNING, LogString : "You don't have hlt at the end of your code. When in Run mode the program might not finish at all"});
    // }

    //force update
    setLine(0)
    loadEmulator(() => {
      setEmulator(new wasm.Emulator(0));
      setPcLineVec(emulator?.load_program(0, code) as Uint32Array);
      console.log(pcLineVec);
      emulator?.set_program_counter(0);
      setLoaded(true);
      enqueueSnackbar('Program Loaded', {
        variant: 'success',
        TransitionComponent: Collapse,
        anchorOrigin: {
          vertical: 'bottom',
          horizontal: 'center',
        }
      });

    });
    stopDecoration();
  }

  function runMode() {
    if (loaded) {
      let c = emulator?.emulate();
    }
  }

  return (

    <div className="mainWindow">
      <Box display={'flex'} alignItems={'center'} justifyContent={'center'} py={2}>
        <Stack
          flexDirection={'row'}
          flexWrap={'wrap'}
          gap={2}
          alignItems={'center'}
          justifyContent={'center'}
        >
          <Button variant="outlined" startIcon={<Build />} onClick={() => assemble()
          }>
            Assemble
          </Button>
          <Button variant={loaded ? "contained" : "outlined"} startIcon={< SimCardDownload />} onClick={() => loadProgram()}>
            {loaded ? "Loaded" : "Load Program"}
          </Button><Button variant="outlined" endIcon={<BugReport />} onClick={() => debugMode()}>
            Debug Mode
          </Button><Button variant="outlined" endIcon={<RunCircle />} onClick={() => runMode()}>
            Run Mode
          </Button>
        </Stack>
      </Box>
      <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
        <Tabs value={tabvalue} onChange={handleChange} centered>
          <Tab label="Code Editor" />
          <Tab label="Flag & Register" />
          <Tab label="Memory" />
        </Tabs>
      </Box>
      <TabPanel value={tabvalue} index={0}>
        <CodeEditor onChange={showValue} />
      </TabPanel>
      <TabPanel value={tabvalue} index={1}>
        {emulator == null ? "loading" : <Flags emulator={emulator as wasm.Emulator} />}
        {emulator == null ? "loading" : <Registers emulator={emulator as wasm.Emulator} />}
      </TabPanel>
      <TabPanel value={tabvalue} index={2}>
        {emulator == null ? "loading" : <MemoryView emulator={emulator as wasm.Emulator} loaded={loaded} />}
      </TabPanel>

    </div>
  );
}

export default App;
