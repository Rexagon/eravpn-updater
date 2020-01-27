import axios from 'axios';
import { parseJwt } from '../stuff/jwt';

const TOKEN_STORAGE_ITEM = 'token';
const TOKEN_TYPE_STORAGE_ITEM = 'token_type';

const ACTION_TYPES = {
  SIGN_IN: 'SIGN_IN',
  SIGN_IN_SUCCESS: 'SIGN_IN_SUCCESS',
  SIGN_IN_FAILURE: 'SIGN_IN_FAILURE',
  SIGN_OUT: 'SIGN_OUT'
};

const initialState = {
  isAuthenticated: false,
  isInProcess: false,
  hasFailure: false
};

export default (state = initialState, action) => {
  switch (action.type) {
    case ACTION_TYPES.SIGN_IN:
      return {
        ...state,
        isAuthenticated: false,
        isInProcess: true,
        hasFailure: false
      };

    case ACTION_TYPES.SIGN_IN_SUCCESS:
      return {
        ...state,
        isAuthenticated: true,
        isInProcess: false,
        hasFailure: false
      };

    case ACTION_TYPES.SIGN_IN_FAILURE:
      return {
        ...state,
        isAuthenticated: false,
        isInProcess: false,
        hasFailure: true
      };

    case ACTION_TYPES.SIGN_OUT:
      return {
        ...state,
        isAuthenticated: false,
        isInProcess: false,
        hasFailure: false
      };

    default:
      return state;
  }
};


export const signIn = data => async dispatch => {
  dispatch({
    type: ACTION_TYPES.SIGN_IN
  });

  try {
    const { token, token_type: tokenType } = (
      await axios.post(`auth/signin`, data)
    ).data;

    axios.defaults.headers['Authorization'] = `${tokenType} ${token}`;
    window.localStorage.setItem(TOKEN_STORAGE_ITEM, token);
    window.localStorage.setItem(TOKEN_TYPE_STORAGE_ITEM, tokenType);

    dispatch({
      type: ACTION_TYPES.SIGN_IN_SUCCESS
    });
  } catch (e) {
    dispatch({
      type: ACTION_TYPES.SIGN_IN_FAILURE
    });
  }
};

export const signOut = () => dispatch => {
  window.localStorage.removeItem(TOKEN_STORAGE_ITEM);
  window.localStorage.removeItem(TOKEN_TYPE_STORAGE_ITEM);

  dispatch({ type: ACTION_TYPES.SIGN_OUT });
};

export const checkAuthentication = () => dispatch => {
  try {
    const token = window.localStorage.getItem(TOKEN_STORAGE_ITEM);
    const tokenType = window.localStorage.getItem(TOKEN_TYPE_STORAGE_ITEM);

    if (typeof token !== 'string' || typeof tokenType !== 'string') {
      throw new Error();
    }

    const jwt = parseJwt(token);
    if (typeof jwt.exp !== 'number' || jwt.exp * 1000 <= new Date().getTime()) {
      throw new Error();
    }

    axios.defaults.headers['Authorization'] = `${tokenType} ${token}`;
    dispatch({
      type: ACTION_TYPES.SIGN_IN_SUCCESS
    });
  } catch (e) {
    dispatch({
      type: ACTION_TYPES.SIGN_OUT
    });
  }
};
