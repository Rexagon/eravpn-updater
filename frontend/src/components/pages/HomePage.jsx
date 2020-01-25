import React from 'react';
import { connect } from 'react-redux';
import { Container, Row, Col, Button } from 'react-bootstrap';

import { signOut } from '../../store/authentication';

const HomePage = ({ signOutConnect }) => (
  <Container fluid={true} className="h-100">
    <Row className="h-100 align-items-center">
      <Col>
        <Row>
          <Col className="text-center mb-5">
            <code>//TODO: make releases list</code>
          </Col>
        </Row>
        <Row>
          <Col className="text-center">
            <Button onClick={signOutConnect}>Sign Out</Button>
          </Col>
        </Row>
      </Col>
    </Row>
  </Container>
);

const mapDispatchToProps = {
  signOutConnect: signOut
};

export default connect(null, mapDispatchToProps)(HomePage);
