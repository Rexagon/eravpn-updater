import { ACTION_TYPES } from '../actions/authentication';

const initialState = {
  isAuthenticated: false,
  isInProcess: false
};

const reducer = (state = initialState, action) => {
  switch (action.type) {
    default:
      return state;
  }
};

export default reducer;
