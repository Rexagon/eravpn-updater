import React from 'react';
import { Container, Row, Col } from 'react-bootstrap';

import SignInForm from '../SignInForm';

const SignIn = () => (
  <Container fluid={true} className="h-100">
    <Row className="h-100 align-items-center">
      <Col xs={12} md={4} lg={3} className="ml-auto mr-auto">
        <SignInForm />
      </Col>
    </Row>
  </Container>
);

export default SignIn;
