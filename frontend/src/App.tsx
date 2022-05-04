import { useEffect, useRef, useState } from 'react';

import Flags from './components/Flags/flags';
import Registers from './components/Register/registers';

import * as wasm from './wasm/wasm_8085';

import { Log, LogType } from './utils/log';
import { Box, Button, Collapse, Stack, Tab, Tabs, } from '@mui/material';
import { BugReport, Build, RunCircle, SimCardDownload } from '@mui/icons-material';
import { TabPanel } from './components/TabPanel/tab_panel';
import { useSnackbar } from 'notistack';
import CodeEditor from './components/Editor/editor';
import MemoryView from './components/MemoryView/memory_view';

function App() {

    const [tabvalue, setTab] = useState(0);

    const handleChange = (event: React.SyntheticEvent, newValue: number) => {
        setTab(newValue);
    };


    let [line, setLine] = useState<number>(1);
    let [log, setLog] = useState<Log>({ type: LogType.NOTHING, LogString: "" })
    let [pcLineVec, setPcLineVec] = useState<Uint32Array>(new Uint32Array());

    let [loaded, setLoaded] = useState<boolean>(false);
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
            }
            ;
        });
    }, []);


    function debugMode() {
        if (loaded) {
            emulator?.emulate_line_by_line();
            let val = pcLineVec.findIndex((val) => {
                return emulator?.program_counter() as number === val;
            });

            setLine(val + 1);
            var newLine = val + 1;
            localStorage.setItem("lineToHighlight", newLine.toString());
            window.dispatchEvent(new Event('highlightChange'));
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
        localStorage.setItem("8085-warning", JSON.stringify({
            "line": linenumber,
            "message": errorString,
        }));
        window.dispatchEvent(new Event("8085-warning"));
    }

    function showError(linenumber: number, errorString: string) {
        localStorage.setItem("8085-error", JSON.stringify({
            "line": linenumber,
            "message": errorString,
        }));
        window.dispatchEvent(new Event("8085-error"));
    }

    function loadProgram() {
        setLine(0)
        let code = localStorage.getItem("code")?.split("\n") ?? [];
        loadEmulator(() => {
            setEmulator(new wasm.Emulator(0));
            try {
                let programCounterToLineNumberMapping = emulator?.load_program(0, code) as Uint32Array;
                setPcLineVec(programCounterToLineNumberMapping);
                if (code.filter(opcode => opcode.trim().toLowerCase() == "hlt").length == 0) {
                    showWarning(code.length, "Program doesn't contain hlt instruction. Program might not halt. Hint: add HLT instruction");
                }
            } catch (err: any) {
                let error = err.split(";");
                let lineNumber = parseInt(error);
                let errorString = error[1];

                showError(lineNumber, errorString);
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
        localStorage.removeItem("8085-error");
        localStorage.removeItem("8085-warning");
        localStorage.removeItem("lineToHighlight");
        window.dispatchEvent(new Event("8085-error"));
        window.dispatchEvent(new Event("8085-warning"));
        window.dispatchEvent(new Event("highlightChange"));

    }

    function runMode() {
        if (loaded) {
            let c = emulator?.emulate();
        }
    }


    function assemble() {

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
                        onClick={() => loadProgram()}>
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
                <CodeEditor changeHandler={() => { setLoaded(false) }} />
            </TabPanel>
            <TabPanel value={tabvalue} index={1}>
                {emulator == null ? "loading" : <Flags emulator={emulator as wasm.Emulator} />}
                {emulator == null ? "loading" : <Registers emulator={emulator as wasm.Emulator} />}
            </TabPanel>
            <TabPanel value={tabvalue} index={2}>
                {emulator == null ? "loading" : <MemoryView emulator={emulator as wasm.Emulator} loaded={false} />}
            </TabPanel>

        </div>
    );
}

export default App;
