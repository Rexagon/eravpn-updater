import React from 'react';
import ReactDOM from 'react-dom';
import { applyMiddleware, createStore } from 'redux';
import thunk from 'redux-thunk';

import rootReducer from './reducers';
import initialState from './initialState';

import App from './App';

import 'bootstrap/dist/css/bootstrap.min.css';
import './styles/index.scss';

const store = createStore(rootReducer, initialState, applyMiddleware(thunk));

ReactDOM.render(<App store={store} />, document.getElementById('root'));
