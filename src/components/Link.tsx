import { invoke } from '@tauri-apps/api/core';
import { MouseEvent, ReactNode } from 'react';

const openUrl = (url: string)=>url && invoke('open_url', {url});

const openInBrowser = (event: MouseEvent<HTMLAnchorElement>)=>{
  event.preventDefault();
  event.stopPropagation();
  const a = event.currentTarget;
  openUrl(a.href);
};

type LinkParams = {
  href: string,
  children: ReactNode,
};

export default (p: LinkParams)=>(
  <a href={p.href} target="_blank" onClick={openInBrowser}>
    {p.children}
  </a>
);
