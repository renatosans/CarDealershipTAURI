import './styles.css'
import App from './App'
import React from 'react'
import ReactDOM from 'react-dom/client'
import Layout from './components/Layout'


ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <Layout>
      <App />
    </Layout>
  </React.StrictMode>
)
