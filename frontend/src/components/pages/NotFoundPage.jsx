import React from 'react';
import { Link } from 'react-router-dom';
import { Row, Col } from 'react-bootstrap';

import CenterLayout from '../layouts/CenterLayout';

const NotFoundPage = () => (
  <CenterLayout lg={5}>
    <Row>
      <Col className="text-center">
        <code className="display-1">404</code>
      </Col>
    </Row>
    <Row>
      <Col className="text-center">
        <code className="display-1">Not Found</code>
        <hr />
      </Col>
    </Row>
    <Row>
      <Col className="text-center">
        <code className="h3">
          <Link to="/">Home</Link>
        </code>
      </Col>
    </Row>
  </CenterLayout>
);

export default NotFoundPage;
