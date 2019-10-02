import * as React from 'react';
import { ReactNode } from 'react';
import { observer } from 'mobx-react';
import { AppStore } from '../models/AppStore';

@observer
class TimerView extends React.Component<{ store: AppStore }> {
  render(): ReactNode {
    return (
      <>
        <pre>Hello, world!</pre>
        {this.props.store.value}
      </>
    );
  }
}

export default TimerView;
