import { combineReducer } from 'redux';

import authentication from './authentication';

const rootReducer = combineReducer({
  authentication
});

export default rootReducer;
