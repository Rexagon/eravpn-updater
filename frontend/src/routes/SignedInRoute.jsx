import React from 'react';
import { connect } from 'react-redux';
import { Route, Redirect } from 'react-router-dom';

const SignedInRoute = ({ component, isAuthenticated, ...rest }) => {
  if (isAuthenticated === false) {
    return <Redirect to="/signin" />;
  }

  const Component = component;

  return (
    <Route {...rest} render={routeProps => <Component {...routeProps} />} />
  );
};

const mapStateToProps = ({ authentication }) => ({
  isAuthenticated: authentication.isAuthenticated
});

export default connect(mapStateToProps)(SignedInRoute);
