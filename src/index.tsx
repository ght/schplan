import 'core-js/stable';
import 'regenerator-runtime/runtime';
import React from 'react';
import ReactDOM from 'react-dom';
import { App } from './components/App';
import { onSnapshot, destroy, getSnapshot } from 'mobx-state-tree';
import { AppStore } from './models/AppStore';

const localStorageKey = 'mst-todomvc-example';
const initialState = localStorage.getItem(localStorageKey)
  ? JSON.parse(localStorage.getItem(localStorageKey))
  : undefined;

let store = AppStore.create(initialState) as AppStore;
let snapshotListener: () => void;
let containerElement: HTMLElement;

function createAppState(snapshot: AppStore | null): AppStore {
  // clean up snapshot listener
  if (snapshotListener) snapshotListener();
  // kill old store to prevent accidental use and run clean up hooks
  if (store) destroy(store);
  // create new one
  store = AppStore.create(snapshot) as AppStore;
  // connect local storage
  snapshotListener = onSnapshot(store, snapshot =>
    localStorage.setItem(localStorageKey, JSON.stringify(snapshot))
  );

  return store;
}

function renderApp(Component: typeof App, store: AppStore): void {
  if (!containerElement) {
    containerElement = document.createElement('div');
    document.body.appendChild(containerElement);
  }

  ReactDOM.render(<Component store={store} />, containerElement);
}

// Initial render
renderApp(App, createAppState(initialState));

// Disable safari overview behavior
document.body.style.overflow = 'hidden';

// Connect HMR
declare const module: NodeModule;

if (module.hot) {
  module.hot.accept(['./models/AppStore'], () => {
    // Store definition changed, recreate a new one from old state
    renderApp(App, createAppState(getSnapshot(store) as AppStore));
  });

  module.hot.accept(['./components/App'], () => {
    // Componenent definition changed, re-render app
    renderApp(App, store);
  });
}
