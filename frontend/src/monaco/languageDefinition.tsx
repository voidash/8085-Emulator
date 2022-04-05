export default function EightOEightFiveDefinition(): any{
let definition8085 = {
          digits: /\d+(?:_\d+)*[dD]?/,
          octaldigits: /[0-7]+(_+[0-7]+)*/,
          binarydigits: /[0-1]+(_+[0-1]+)*/,
          hexdigits: /[[0-9a-fA-F]+(_+[0-9a-fA-F]+)*/,
    registers: [],
          keywords: ['di','sim','lhld','stc','aci','jnz','rm','mvi','rp','inr','hlt','rc','ani','xra','cc','nop','push','adc','add','ral','daa','rpe','cz','cmc','rst','xchg','rz','pop','jnc','rnc','rar','sta','rpo','xthl','call','cnc','in','cnz','dad','ei','mov','jmp','sbb','ora','pchl','ana','lda','sphl','cp','cma','cmp','xri','ori','cpi','jc','sub','dcr','cpe','rim','shld','cm','rrc','sbi','jpe','dcx','jz','rlc','ret','lxi','cpo','jpo','ldax','jp','inx','out','jm','sui','adi','stax','rnz','DI','SIM','LHLD','STC','ACI','JNZ','RM','MVI','RP','INR','HLT','RC','ANI','XRA','CC','NOP','PUSH','ADC','ADD','RAL','DAA','RPE','CZ','CMC','RST','XCHG','RZ','POP','JNC','RNC','RAR','STA','RPO','XTHL','CALL','CNC','IN','CNZ','DAD','EI','MOV','JMP','SBB','ORA','PCHL','ANA','LDA','SPHL','CP','CMA','CMP','XRI','ORI','CPI','JC','SUB','DCR','CPE','RIM','SHLD','CM','RRC','SBI','JPE','DCX','JZ','RLC','RET','LXI','CPO','JPO','LDAX','JP','INX','OUT','JM','SUI','ADI','STAX','RNZ','a', 'b', 'c', 'd', 'e', 'h', 'l', 'm', 'psw','A', 'B', 'C', 'D', 'E', 'H', 'L', 'M', 'PSW', ],
          tokenizer: {
            root: [
              [/[a-zA-Z][\w]*/, {
                  cases: {
                    '@keywords': 'keyword',
                    '@default': 'identifier',
                  }
              }],

              [/(@octaldigits)[qQ]/, 'number.octal'],
              [/(@binarydigits)[bB]/, 'number.binary'],
              [/0x(@hexdigits)[hH]?/, 'number.hexdigits'],
              [/@digits/, 'number'],
              {include: '@whitespace'}
            ],

            whitespace: [
              [/[ \t\r\n]+/, ''],
              [/;.*$/, 'comment']
            ],
          },
};
  return definition8085;
}
