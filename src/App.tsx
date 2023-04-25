import {useEffect, useState} from 'react';
import {DateTime} from 'luxon';

function App() {
  const [time, setTime] = useState('');

  const update = async () => setTime(DateTime.now().toFormat('HH:mm:ss.uuu'));

  useEffect(() => {
    update();
    const id = setInterval(update, 50);

    return () => clearInterval(id);
  }, []);

  return (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 22">
      <text x="50%" y="50%" dominant-baseline="middle" text-anchor="middle">{time}</text>
    </svg>
  );
}

export default App;
