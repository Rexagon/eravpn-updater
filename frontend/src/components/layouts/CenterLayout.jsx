import React from 'react';
import { Container, Row, Col } from 'react-bootstrap';

const CenterLayout = ({ children, ...rest }) => (
  <Container fluid={true} className="h-100">
    <Row className="h-100 align-items-center">
      <Col xs={12} md={4} lg={3} {...rest} className="ml-auto mr-auto">
        {children}
      </Col>
    </Row>
  </Container>
);

export default CenterLayout;
