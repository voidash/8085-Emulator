import { useEffect, useRef, useState } from 'react';

import Flags from './components/Flags/flags';
import Registers from './components/Register/registers';
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
import { Range } from 'monaco-editor/esm/vs/editor/editor.api';
import * as wasm from './wasm/wasm_8085';
import Editor, { Monaco, useMonaco } from "@monaco-editor/react";
import { Log, LogType } from './utils/log';
import { Box, Button, Chip, Collapse, Divider, Stack, Tab, Tabs, } from '@mui/material';
import { BugReport, Build, RunCircle, SimCardDownload } from '@mui/icons-material';
import { TabPanel } from './components/TabPanel/tab_panel';
import { useSnackbar } from 'notistack';
import CodeEditor from './components/Editor/editor';
import MemoryView from './components/MemoryView/memory_view';

import EightOEightFiveDefinition from "./monaco/languageDefinition";

function App() {

    const [tabvalue, setTab] = useState(0);

    const handleChange = (event: React.SyntheticEvent, newValue: number) => {
        setTab(newValue);
    };

    let [assembled, setAssembled] = useState<string>("<span class='comment'>Assemble First</span>");
    let [code, setCode] = useState<string[]>(localStorage.getItem("code")?.split("\n") || []);
    const editorRef = useRef<CodeEditor | null>(null);
    let [line, setLine] = useState<number>(1);
    let [log, setLog] = useState<Log>({ type: LogType.NOTHING, LogString: "" })
    let [pcLineVec, setPcLineVec] = useState<Uint32Array>(new Uint32Array());
    let [loaded, setLoaded] = useState<boolean>(false);
    let [emulator, setEmulator] = useState<wasm.Emulator | null>(null);
    let [decoration, setDecoration] = useState<string[] | undefined>();

    const { enqueueSnackbar } = useSnackbar();


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
            }
            ;
        });
    }, []);

  // load monaco
  useEffect(() => {
      if (monaco) {
        monaco.languages.register({id: '8085asm'});
        monaco.languages.setMonarchTokensProvider('8085asm', EightOEightFiveDefinition());
      }
  }, [monaco]); 


    function debugMode() {
        if (loaded) {
            emulator?.emulate_line_by_line();
            let val = pcLineVec.findIndex((val) => {
                return emulator?.program_counter() as number === val;
            });
            setLine(val + 1);
            var newLine = val + 1;
            gotoLine(val+1);
            stopDecoration();
            localStorage.setItem("lineToHighlight", newLine.toString());
            //window.dispatchEvent(new Event('highlightChange'));
        } else {
            enqueueSnackbar('Please load program first.', {
                variant: 'warning',
                TransitionComponent: Collapse,
                anchorOrigin: {
                    vertical: 'bottom',
                    horizontal: 'center',
                }
            });
        }
    }
 function showWarning(linenumber: number, errorString: string) {
    monaco?.editor.setModelMarkers(editorRef.current?.getModel() as monaco.editor.ITextModel, '8085-warning', [{
      startLineNumber: linenumber,
      startColumn: 1,
      endLineNumber: 2,
      endColumn: 1000,
      message: errorString,
      severity: monaco.MarkerSeverity.Warning
    }])
  }

  function showError(linenumber: number, errorString: string) {
    monaco?.editor.setModelMarkers(editorRef.current?.getModel() as monaco.editor.ITextModel, '8085-error', [{
      startLineNumber: linenumber,
      startColumn: 1,
      endLineNumber: 2,
      endColumn: 1000,
      message: errorString,
      severity: monaco.MarkerSeverity.Error
    }])
  }

    function loadProgram() {
        setTab(0);
        if(!loaded && tabvalue === 0) {
        onChange();
        setLine(0);
        let code = localStorage.getItem("code")?.split("\n") ?? [];
        loadEmulator(() => {
            setEmulator(new wasm.Emulator(0));
            try {
                let programCounterToLineNumberMapping = emulator?.load_program(0, code) as Uint32Array;
                setPcLineVec(programCounterToLineNumberMapping);
                console.log("mapping is " +  programCounterToLineNumberMapping);
                if (code.filter(opcode => opcode.trim().toLowerCase() === "hlt").length === 0) {
                    showWarning(code.length, "Program doesn't contain hlt instruction. Program might not halt. Hint: add HLT instruction");
                }
            } catch (err: any) {
                let error = err.split(";");
                let lineNumber = parseInt(error);
                let errorString = error[1];

                showError(lineNumber, errorString);

                enqueueSnackbar(`Program couldnt load due to error on line ${lineNumber}`, {
                    variant: 'error',
                    TransitionComponent: Collapse,
                    anchorOrigin: {
                        vertical: 'bottom',
                        horizontal: 'center',
                    }
                });
            return;
            }

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

        localStorage.removeItem("8085-error");
        localStorage.removeItem("8085-warning");
        localStorage.removeItem("lineToHighlight");
        window.dispatchEvent(new Event("8085-error"));
        window.dispatchEvent(new Event("8085-warning"));
        window.dispatchEvent(new Event("highlightChange"));

        }else {
            setLoaded(false);
        }
    }

    function runMode() {
        if (loaded && code.length > 1) {
           if(code[code.length - 1].trim().startsWith("hlt")) {
                emulator?.emulate();
                let val = pcLineVec.findIndex((val) => {
                    return emulator?.program_counter() as number === val;
                });
                setLine(val + 1);
                var newLine = val + 1;
                gotoLine(val+1);
                stopDecoration();
           }else {
            enqueueSnackbar('Code doesn\'t contain hlt instruction for Run mode. add "hlt" at last line', {
                variant: 'error',
                TransitionComponent: Collapse,
                anchorOrigin: {
                    vertical: 'bottom',
                    horizontal: 'center',
                }
            });
           }
        } 

        else {
            enqueueSnackbar('Please load program first.', {
                variant: 'warning',
                TransitionComponent: Collapse,
                anchorOrigin: {
                    vertical: 'bottom',
                    horizontal: 'center',
                }
            });
        }
    }


    function assemble() {
        onChange();
        setTab(1);
        let assembledString =  "";
        code.forEach((c) => {
            let data = emulator?.get_assembly([c] as any);
            console.log(data);
            let assembledData = "" ;
            data?.forEach((d) => {
                assembledData +=" <span class='hex'>" + String(d) + "</span>" ;
            });
            assembledData += " <span class='comment'>;" + c + "</span><br/>";
            assembledString += assembledData;
        });
        setAssembled(assembledString);
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
      if(decoration){
        editorRef.current?.deltaDecorations(decoration as string[], []);
      }
        monaco?.editor.setModelMarkers(editorRef.current?.getModel() as monaco.editor.ITextModel, '8085-warning', []);
        monaco?.editor.setModelMarkers(editorRef.current?.getModel() as monaco.editor.ITextModel, '8085-error', []);
  }


    var onChange = () => {
        setLoaded(false);
        let codeBuffer: String = editorRef.current?.getValue() as String;
        setCode(codeBuffer.split("\n"));
        localStorage.setItem("code", codeBuffer.toString());
        stopDecoration();
    }

    function handleEditorDidMount(editor: monaco.editor.IStandaloneCodeEditor, _: Monaco) {
        console.log("editor mounted");
        editorRef.current = editor;
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
                    <Button variant={loaded ? "contained" : "outlined"} startIcon={< SimCardDownload />}
                        onClick={() =>  loadProgram()}>
                        {loaded ? "Loaded" : "Load Program"}
                    </Button><Button variant="outlined" endIcon={<BugReport />} onClick={() => debugMode()}>
                        Debug Mode
                    </Button><Button variant="outlined" endIcon={<RunCircle />} onClick={() => runMode()}>
                        Run Mode
                    </Button>
                </Stack>
            </Box>
            <Box className='desktopView'>
                <Box display={"flex"} justifyContent={"space-between"}>
                    <Box className='editor'>
                    <Tabs value={tabvalue} onChange={handleChange} centered>
                        <Tab label="Editor" />
                        <Tab label="Assembled Code" />
                    </Tabs>

                <TabPanel value={tabvalue} index={0}>
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
                    </TabPanel>
                    <TabPanel value={tabvalue} index={1}>
                        <div className="assembledBlock" dangerouslySetInnerHTML ={{__html: assembled as string}} />
                    </TabPanel>
                    </Box>
                    <Box className='information'>
                        {emulator == null ? "loading" : <Flags emulator={emulator as wasm.Emulator} />}
                        {emulator == null ? "loading" : <Registers emulator={emulator as wasm.Emulator} />}
                    </Box>
                </Box>
                <Box height="30px"></Box>
                <Divider>
                    <Chip label="Memory View" />
                </Divider>
                {emulator == null ? "loading" : <MemoryView emulator={emulator as wasm.Emulator} loaded={loaded} />}
            </Box>
            <Box className='mobileView'>
                <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
                    <Tabs value={tabvalue} onChange={handleChange} centered>
                        <Tab label="Code Editor" />
                        <Tab label="Flag & Register" />
                        <Tab label="Memory" />
                    </Tabs>
                </Box>
                <TabPanel value={tabvalue} index={0}>
                    {/*<CodeEditor  dec={decoration as string[]} setDec={(value: string[]) => {setDecoration(value)}} changeHandler={() => { setLoaded(false) }} />*/}
                </TabPanel>
                <TabPanel value={tabvalue} index={1}>
                    {emulator == null ? "loading" : <Flags emulator={emulator as wasm.Emulator} />}
                    {emulator == null ? "loading" : <Registers emulator={emulator as wasm.Emulator} />}
                </TabPanel>
                <TabPanel value={tabvalue} index={2}>
                    {emulator == null ? "loading" : <MemoryView emulator={emulator as wasm.Emulator} loaded={loaded} />}
                </TabPanel>
            </Box>
        </div >
    );
    }

export default App;
