export function name() {
  return 'Rust';
}

export class Greeting {
  constructor(msg, recipient) {
    this.msg = msg;
    this.recipient = recipient;
  }

  greet() {
    console.log("Greeting")  
    console.log(`${this.msg} ${this.recipient}`);
  }
}