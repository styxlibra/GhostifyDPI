import { ReactElement, useEffect, useState } from 'react';
import LaunchStages from '../process/LaunchStages';
import processStatus from '../process/ProcessStatus';
import './StatusMessage.less';
import Link from './Link';

const statusTitle = (stage: LaunchStages): string=>{
  switch (stage) {
    case LaunchStages.Started: return 'Started';
    case LaunchStages.Starting: return 'Starting...';
    case LaunchStages.Stopped: return 'Stopped';
    case LaunchStages.Stopping: return 'Stopping...';
  }
};

const startedDescription = (
  <span>
    Well done!<hr/>
    Now you can close this window (it will be minimized to tray) and visit
    {' '}
    <Link href="https://www.youtube.com">YouTube</Link>
  </span>
);

const stoppedDescription = (
  <span>
    Press the Moon button to fix broken Internet access!
  </span>
);

const description = (stage: LaunchStages): ReactElement=>{
  switch (stage) {
    case LaunchStages.Started: return startedDescription;
    case LaunchStages.Stopped: return stoppedDescription;
  }
  return <></>;
};

export default ()=>{
  const [launchStage, setLaunchStage] = useState(LaunchStages.Stopped);
  useEffect(()=>{
    processStatus.addListener(setLaunchStage);
    return ()=>void processStatus.removeListener(setLaunchStage);
  }, []);
  return (
    <div className="description-container">
      <p className="status-title">{statusTitle(launchStage)}</p>
      <p>{description(launchStage)}</p>
    </div>
  );
};
