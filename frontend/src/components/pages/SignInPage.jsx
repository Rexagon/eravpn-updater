import React from 'react';
import { connect } from 'react-redux';
import { Row, Col } from 'react-bootstrap';

import { signIn } from '../../store/authentication';

import SignInForm from '../forms/SignInForm';
import CenterLayout from '../layouts/CenterLayout';

const SignInPage = ({ signInConnect, isInProcess }) => (
  <CenterLayout>
    <Row>
      <Col className="text-center">
        <h1>
          <b>EraVPN Updater</b>
        </h1>
        <hr />
      </Col>
    </Row>
    <Row>
      <Col>
        <SignInForm onSubmit={signInConnect} disabled={isInProcess} />
      </Col>
    </Row>
  </CenterLayout>
);

const mapStateToProps = ({ authentication }) => ({
  isInProcess: authentication.isInProcess
});

const mapDispatchToProps = {
  signInConnect: signIn
};

export default connect(mapStateToProps, mapDispatchToProps)(SignInPage);
