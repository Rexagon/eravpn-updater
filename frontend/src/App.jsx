import React from 'react';
import PropTypes from 'prop-types';
import { Provider } from 'react-redux';
import { BrowserRouter as Router, Route } from 'react-router-dom';

import LoginPage from './pages/LoginPage';

const App = ({ store }) => (
  <Provider store={store}>
    <Router>
      <Route path="/">
        <LoginPage />
      </Route>
    </Router>
  </Provider>
);

App.propTypes = {
  store: PropTypes.object.isRequired
};

export default App;
