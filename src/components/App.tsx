import * as React from 'react';
import TimerView from './TimerView';
import { AppStore } from '../models/AppStore';

export class App extends React.Component<{ store: AppStore }> {
  render(): React.ReactNode {
    return (
      <>
        <TimerView store={this.props.store} />
      </>
    );
  }
}
