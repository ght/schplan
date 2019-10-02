import { Instance, types } from 'mobx-state-tree';

export const AppStore = types
  .model({
    value: 1
  })
  .actions(self => ({
    setValue(value: number): void {
      self.value = value;
    }
  }))
  .actions(self => {
    let intervalId: NodeJS.Timeout;

    return {
      afterCreate(): void {
        console.log('afterCreate');
        intervalId = setInterval(() => self.setValue(self.value + 1), 1000);
      },
      beforeDestroy(): void {
        console.log('beforeDestroy');
        clearInterval(intervalId);
      }
    };
  });

export type AppStore = Instance<typeof AppStore>;
