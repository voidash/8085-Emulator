export enum LogType {
  WARNING,
  ERROR,
  NOTHING
};

export interface Log {
  type : LogType,
  LogString: string
};

export default function Logger(log: Log) {
  return (
    <div className="log">
      <div className="type">{log.type}</div>
      <div className="logString">{log.LogString}</div>
    </div>
  );
}


