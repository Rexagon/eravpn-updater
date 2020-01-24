import React from 'react';
import { connect } from 'react-redux';
import { Route } from 'react-router-dom';

import history from '../history';

const SignedInRoute = ({ component, isAuthenticated, ...otherProps }) => {
  if (isAuthenticated === false) {
    history.push('/signin');
  }

  return (
    <>
      <header>Signed in header</header>
      <Route render={otherProps => <Component {...otherProps} />} />
      <footer>Signed in footer</footer>
    </>
  );
};

const mapStateToProps = ({ isAuthenticated }) => ({
  isAuthenticated
});

export default connect(mapStateToProps)(SignedInRoute);
