import React, { useEffect } from 'react';
import { connect } from 'react-redux';
import { Container, Row, Col, Button } from 'react-bootstrap';

import { signOut } from '../../store/authentication';
import { fetchReleases } from '../../store/releases';

const HomePage = ({ releases, signOutConnect, fetchReleasesConnect }) => {
  useEffect(() => {
    fetchReleasesConnect();
  }, []);

  return (
    <Container fluid={true} className="h-100">
      <Row className="h-100 align-items-center">
        <Col>
          <Row>
            <Col className="text-center mb-5">
              {releases.map(release => (
                <Row>
                  <Col>
                    <pre>{JSON.stringify(release, undefined, 4)}</pre>
                  </Col>
                </Row>
              ))}
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
};

const mapStateToProps = ({ releases }) => ({
  releases: releases.items
});

const mapDispatchToProps = {
  signOutConnect: signOut,
  fetchReleasesConnect: fetchReleases
};

export default connect(mapStateToProps, mapDispatchToProps)(HomePage);
