import React from 'react';
import { connect } from 'react-redux';

import { signIn } from '../../store/authentication';

import SignInForm from '../forms/SignInForm';
import CenterLayout from '../layouts/CenterLayout';

const SignInPage = ({ signInConnect, isInProcess }) => (
  <CenterLayout>
    <SignInForm onSubmit={signInConnect} disabled={isInProcess} />
  </CenterLayout>
);

const mapStateToProps = ({ authentication }) => ({
  isInProcess: authentication.isInProcess
});

const mapDispatchToProps = {
  signInConnect: signIn
};

export default connect(mapStateToProps, mapDispatchToProps)(SignInPage);
