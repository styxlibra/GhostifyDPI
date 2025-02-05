import { Dispatch, SetStateAction, useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import powerSymbol from '../assets/power.svg';
import LaunchStages from '../process/LaunchStages';
import processStatus from '../process/ProcessStatus';
import './ActivateButton.less';

const buttonAlt = (stage: LaunchStages): string=>{
  switch (stage) {
    case LaunchStages.Started: return 'Turn off';
    case LaunchStages.Starting: return 'Starting...';
    case LaunchStages.Stopped: return 'Turn on';
    case LaunchStages.Stopping: return 'Stopping...';
  }
};

const start = async (setLaunchStage: Dispatch<SetStateAction<LaunchStages>>)=>{
  setLaunchStage(LaunchStages.Starting);
  await invoke('start');
};

const stop = async (setLaunchStage: Dispatch<SetStateAction<LaunchStages>>)=>{
  setLaunchStage(LaunchStages.Stopping);
  await invoke('stop');
};

const activate = (launchStage: LaunchStages, setLaunchStage: Dispatch<SetStateAction<LaunchStages>>)=>{
  if (launchStage==LaunchStages.Stopped)
    return start(setLaunchStage);
  if (launchStage==LaunchStages.Started)
    return stop(setLaunchStage);
};

export default ()=>{
  const [launchStage, setLaunchStage] = useState(LaunchStages.Stopped);
  useEffect(()=>{
    processStatus.addListener(setLaunchStage);
    return ()=>void processStatus.removeListener(setLaunchStage);
  }, []);
  const launchStageClass = LaunchStages[launchStage];
  const isDisabled = launchStage==LaunchStages.Starting || launchStage==LaunchStages.Stopping;
  const onClick = ()=>activate(launchStage, setLaunchStage);
  return (
    <button className={`activate-button ${launchStageClass}`} onClick={onClick} disabled={isDisabled}>
      <img src={powerSymbol} alt={buttonAlt(launchStage)} className="power-logo"/>
    </button>
  );
};
