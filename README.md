# CarDealershipTAURI
Sistema para concessionária de carros utilizando REACT e RUST

Monolito "fuderoso" utilizado para prototipagem de MVPs com REACT no frontend e RUST no backend.

Para o produto subir em produção separar frontend e backend em 2 repositórios (ou monorepo)
O mesmo se aplica para escalar o time de desenvolvimento, com 2 equipes, uma para evoluir o frontend e e outra para evoluir o backend de forma separada.

## Steps to run the project
- run the sql script to create the database
- .env
- npm install
- npm run tauri dev
- Frontend at http://127.0.0.1:1420/
- Backend at http://127.0.0.1:8080/api/cars
