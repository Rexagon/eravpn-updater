import axios from 'axios';
import { API_URL } from '../../constants';

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

  const response = (await axios.post(`${API_URL}/auth/signin`, data)).data;

  if (response.isSuccess === true) {
    axios.defaults.headers[
      'Authorization'
    ] = `${response.data.token_type} ${response.data.token}`;

    dispatch({
      type: ACTION_TYPES.SIGN_IN_SUCCESS
    });
  } else {
    dispatch({
      type: ACTION_TYPES.SIGN_IN_FAILURE
    });
  }
};

export const signOut = () => ({ type: ACTION_TYPES.SIGN_OUT });
