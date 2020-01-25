import React from 'react';
import { Route, Switch } from 'react-router-dom';

import HomePage from '../components/pages/HomePage';
import SignInPage from '../components/pages/SignInPage';
import NotFoundPage from '../components/pages/NotFoundPage';

import SignedInRoute from './SignedInRoute';
import SignedOutRoute from './SignedOutRoute';

const Pages = () => (
  <Switch>
    <SignedInRoute path="/" exact={true} component={HomePage} />

    <SignedOutRoute path="/signin" exact={true} component={SignInPage} />

    <Route component={NotFoundPage} />
  </Switch>
);

export default Pages;
