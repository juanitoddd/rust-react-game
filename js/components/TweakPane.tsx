import { useEffect } from "react";
import {Pane} from 'tweakpane';
import { set_line_color } from 'gamelib';

function TweakPane() {   
  useEffect(() => {    
    const PARAMS = {
      // factor: 123,
      // title: 'hello',
      color: '#888',
      // percentage: 50,
      // theme: 'dark',
    };

    const pane = new Pane();

    // pane.addBinding(PARAMS, 'factor');
    // pane.addBinding(PARAMS, 'title');
    const colorPane = pane.addBinding(PARAMS, 'color');
    colorPane.on('change', (ev) => {
      console.log("🚀 ~ ev.value:", ev.value)
      set_line_color(ev.value);
    }
    )
    // const b = pane.addBinding(
    //   PARAMS, 'percentage',
    //   {min: 0, max: 100, step: 10}
    // );
    // b.on('change', function(ev) {      
    //   set_param(ev.value);
    // });

    // `options`: list
    // pane.addBinding(
    //   PARAMS, 'theme',
    //   {options: {Dark: 'dark', Light: 'light'}}
    // );


  }, [])  

  return null
}

export default TweakPane