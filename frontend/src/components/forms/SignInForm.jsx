import React, { useState } from 'react';
import { Form, Button } from 'react-bootstrap';

const SignInForm = ({ onSubmit, disabled }) => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  return (
    <Form
      onSubmit={event => {
        event.preventDefault();
        onSubmit({
          username,
          password
        });
      }}
    >
      <Form.Group controlId="username">
        <Form.Label>Username</Form.Label>
        <Form.Control
          placeholder="Enter username"
          disabled={disabled}
          value={username}
          onChange={event => setUsername(event.target.value)}
        />
      </Form.Group>
      <Form.Group controlId="password">
        <Form.Label>Password</Form.Label>
        <Form.Control
          placeholder="Enter password"
          type="password"
          disabled={disabled}
          value={password}
          onChange={event => setPassword(event.target.value)}
        />
      </Form.Group>
      <Button
        variant="primary"
        type="submit"
        className="w-100"
        disabled={disabled}
      >
        Sign In
      </Button>
    </Form>
  );
};

export default SignInForm;
