import React, { useEffect } from 'react';
import { connect } from 'react-redux';
import { BrowserRouter as Router, Route } from 'react-router-dom';

import { checkAuthentication } from './store/authentication';

import Pages from './routes/Pages';

const App = ({ checkAuthenticationConnect }) => {
  useEffect(() => checkAuthenticationConnect());

  return (
    <Router>
      <Route component={Pages} />
    </Router>
  );
};

const mapDispatchToProps = {
  checkAuthenticationConnect: checkAuthentication
};

export default connect(null, mapDispatchToProps)(App);
