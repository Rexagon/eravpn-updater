import { applyMiddleware, combineReducers, createStore } from 'redux';
import thunk from 'redux-thunk';

import authentication from './authentication';
import releases from './releases';

const rootReducer = combineReducers({
  authentication,
  releases
});

const middleware = [thunk];

const store = createStore(
  rootReducer,
  undefined,
  applyMiddleware(...middleware)
);

export default store;
