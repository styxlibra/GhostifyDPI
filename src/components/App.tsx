import processStatus from '../process/ProcessStatus';
import ActivateButton from './ActivateButton';
import StatusMessage from './StatusMessage';
import './App.less';

const AppTitle = ()=>{
  const appTitle = 'Ghostify DPI';
  return (
    <div className="app-title">{appTitle}</div>
  );
};

export default ()=>{
  processStatus.subscribe();
  return (
    <div className="container">
      <AppTitle/>
      <ActivateButton/>
      <StatusMessage/>
    </div>
  );
};
