import axios from 'axios';

export const ACTION_TYPES = {
  SIGN_IN: 'SIGN_IN',
  SIGN_IN_SUCCESS: 'SIGN_IN_SUCCESS',
  SIGN_IN_FAILURE: 'SIGN_IN_FAILURE',
  SIGN_OUT: 'SIGN_OUT'
};

export const signIn = data => async dispatch => {
  dispatch({
    type: ACTION_TYPES.SIGN_IN
  });

  try {
    const response = (await axios.post(`auth/signin`, data)).data;

    axios.defaults.headers[
      'Authorization'
    ] = `${response.data.token_type} ${response.data.token}`;

    dispatch({
      type: ACTION_TYPES.SIGN_IN_SUCCESS
    });
  } catch (e) {
    dispatch({
      type: ACTION_TYPES.SIGN_IN_FAILURE
    });
  }
};

export const signOut = () => ({ type: ACTION_TYPES.SIGN_OUT });

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
