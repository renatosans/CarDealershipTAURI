import Draggable from 'react-draggable'
import { carType } from './utils/types'
import { useState, useEffect } from 'react'
import Carousel from './components/Carousel'
import styles from '@/styles/Home.module.css'
import VehicleForm from './components/VehicleForm'
import VehicleList from './components/VehicleList'
import toast, { Toaster } from "react-hot-toast"


export default function Home() {
  const [open, setOpen] = useState(false);
  const [cars, setCars] = useState<carType[]>();

  useEffect(() => {
    getCars();
  }, []);

  const getCars = () => {
    fetch('api/cars')
    .then(resp => resp.json())
    .then(resultSet => setCars(resultSet))
    .catch(error => console.error(error))
  }

  const toggle = () => {
    setOpen(current => !current);
  }

  const addCar = () => {
    setOpen(true);
  }

  const addCustomer = () => {
    // not implemented yet
  }

  return (
    <div className={styles.container}>
      <header>
        <title>Car Dealership</title>
        <meta name="description" content="Ride your own car" />
        <link rel="icon" href="/favicon.ico" />
      </header>
      <Toaster/>
      <Carousel/>
      <Draggable>
      <div>
        <VehicleForm parentRef={{ toggle, getCars }} opened={open} />
      </div>
      </Draggable>
      <div className={styles.actions}>
        <button className={styles.button} onClick={addCar}>Cadastrar veículo</button>        
        <button className={styles.button} onClick={addCustomer}>Cadastrar Cliente</button>
      </div>
      <VehicleList items={cars} desc={'Car for sale. Available'} />
    </div>
  )
}
