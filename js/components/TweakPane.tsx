import { useEffect } from "react";
import {Pane} from 'tweakpane';
import { get_info, set_param } from 'gamelib';

function TweakPane() {   
  useEffect(() => {
    const PARAMS = {
      factor: 123,
      title: 'hello',
      color: '#ff0055',
      percentage: 50,
      theme: 'dark',
    };

    const pane = new Pane();

    pane.addBinding(PARAMS, 'factor');
    pane.addBinding(PARAMS, 'title');
    pane.addBinding(PARAMS, 'color');
    const b = pane.addBinding(
      PARAMS, 'percentage',
      {min: 0, max: 100, step: 10}
    );
    b.on('change', function(ev) {      
      set_param(ev.value);
    });

    // `options`: list
    pane.addBinding(
      PARAMS, 'theme',
      {options: {Dark: 'dark', Light: 'light'}}
    );
  }, [])  

  return null
}

export default TweakPane