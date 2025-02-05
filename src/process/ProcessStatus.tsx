import { EventEmitter, Listener } from 'events';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import LaunchStages from './LaunchStages';

const events = new EventEmitter();
const EVENT_NAME = 'statusChange';
let initialized = false;
let currentStatus = LaunchStages.Stopped;

const subscribe = async ()=>{
  if (initialized)
    return;
  initialized = true;
  await listen<string>('status', event=>updateStatus(event.payload));
  const status = await invoke<string>('status');
  updateStatus(status);
};

const updateStatus = (statusName: string)=>{
  currentStatus = LaunchStages[statusName as keyof typeof LaunchStages];
  events.emit(EVENT_NAME, currentStatus);
};

const addListener = (listener: Listener)=>events.addListener(EVENT_NAME, listener);

const removeListener = (listener: Listener)=>events.removeListener(EVENT_NAME, listener);

export default {subscribe, addListener, removeListener};
