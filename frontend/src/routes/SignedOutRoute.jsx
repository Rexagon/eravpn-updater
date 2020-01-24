import React from 'react';
import { connect } from 'react-redux';
import { Route } from 'react-router-dom';

import history from '../history';

const SignedOutRoute = ({ component, isAuthenticated, ...otherProps }) => {
  if (isAuthenticated === true) {
    history.push('/home');
  }

  return (
    <>
      <header>Signed out header</header>
      <Route render={otherProps => <Component {...otherProps} />} />
      <footer>Signed out footer</footer>
    </>
  );
};

const mapStateToProps = ({ isAuthenticated }) => ({
  isAuthenticated
});

export default connect(mapStateToProps)(SignedOutRoute);
