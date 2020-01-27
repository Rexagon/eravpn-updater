import axios from 'axios';

const ACTION_TYPES = {
  RELEASES_FETCH: 'RELEASES_FETCH',
  RELEASES_FETCH_SUCCESS: 'RELEASES_FETCH_SUCCESS',
  RELEASES_FETCH_FAILURE: 'RELEASES_FETCH_FAILURE'
};

const initialState = {
  items: [],
  isInProcess: false,
  hasFailure: false
};

export default (state = initialState, action) => {
  switch (action.type) {
    case ACTION_TYPES.RELEASES_FETCH:
      return {
        ...state,
        isInProcess: true,
        hasFailure: false
      };

    case ACTION_TYPES.RELEASES_FETCH_SUCCESS:
      return {
        ...state,
        items: action.payload,
        isInProcess: false,
        hasFailure: false
      };

    case ACTION_TYPES.RELEASES_FETCH_FAILURE:
      return {
        ...state,
        isInProcess: false,
        hasFailure: false
      };

    default:
      return state;
  }
};

export const fetchReleases = () => async dispatch => {
  dispatch({
    type: ACTION_TYPES.RELEASES_FETCH
  });

  try {
    const releases = (await axios('http://127.0.0.1:10000/api/releases')).data;

    dispatch({
      type: ACTION_TYPES.RELEASES_FETCH_SUCCESS,
      payload: releases
    });
  } catch (e) {
    dispatch({
      type: ACTION_TYPES.RELEASES_FETCH_FAILURE
    });
  }
};
