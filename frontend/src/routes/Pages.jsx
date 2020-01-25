import React from 'react';
import { Route, Switch } from 'react-router-dom';

import SignInPage from '../components/pages/SignInPage';
import NotFoundPage from '../components/pages/NotFoundPage';

import SignedInRoute from './SignedInRoute';
import SignedOutRoute from './SignedOutRoute';

const Pages = () => (
  <Switch>
    <SignedOutRoute path="/" exact={true} component={SignInPage} />
    <Route component={NotFoundPage} />
  </Switch>
);

export default Pages;
