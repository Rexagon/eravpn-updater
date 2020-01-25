import React from 'react';
import { BrowserRouter as Router, Route } from 'react-router-dom';

import Pages from './routes/Pages';

const App = () => (
  <Router>
    <Route component={Pages} />
  </Router>
);

export default App;
