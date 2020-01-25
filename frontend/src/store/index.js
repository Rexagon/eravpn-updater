import { applyMiddleware, combineReducers, createStore } from 'redux';
import thunk from 'redux-thunk';

import authentication from './authentication';

const rootReducer = combineReducers({
  authentication
});

const middleware = [thunk];

const store = createStore(
  rootReducer,
  undefined,
  applyMiddleware(...middleware)
);

export default store;
