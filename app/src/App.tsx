import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';
import './App.css';

export default function App() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const login = () => {
    invoke('login_user', {
      email,
      password,
    })
      .then((uid) => {
        console.log(uid);
      })
      .catch((err) => {
        alert(err);
      });
  };

  const signUp = () => {
    invoke('sign_up_user', {
      email,
      password,
    })
      .then((uid) => {
        alert(String(uid));
      })
      .catch((err) => {
        alert(err);
      });
  };

  return (
    <div>
      <div>
        <input
          type='email'
          placeholder='Email'
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
        <input
          type='password'
          placeholder='Password'
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        <button type='submit' onClick={signUp}>
          Sign Up
        </button>
        <button type='submit' onClick={login}>
          Login
        </button>
      </div>
    </div>
  );
}
