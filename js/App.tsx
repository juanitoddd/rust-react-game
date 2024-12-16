import { useEffect, useState } from 'react'
import { useWindowSize } from '@uidotdev/usehooks';
import TweakPane from './components/TweakPane';
import init, { start_game, greet } from 'gamelib';
import { Greeting } from './greeting.js'
import './App.css'

let initialized: boolean = false;

declare global {
  interface Window { greeting: any; }
}

function App() {
  const size = useWindowSize();    
  useEffect(() => {
    window.greeting = new Greeting("Hello", "World");
    init().then(() => {
      if(!initialized) {
        greet("greeting", "greet")
        start_game();                
        initialized = true;
      }
    })
  }, [])

  return (
    <>
      <TweakPane />
      <canvas id="canvas" height={size.height || 0} width={size.width || 0} />
    </>
  )
}

export default App
