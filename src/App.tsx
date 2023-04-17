import "./App.css";
import Draggable from 'react-draggable'
import { carType } from './utils/types'
import { useState, useEffect } from 'react'
import Carousel from './components/Carousel'
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
    <div className="container">
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
      <div className="actions">
        <button className="button" onClick={addCar}>Cadastrar veículo</button>        
        <button className="button" onClick={addCustomer}>Cadastrar Cliente</button>
      </div>
      <VehicleList items={cars} desc={'Car for sale. Available'} />
    </div>
  )
}
