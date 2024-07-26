// import { $router } from './stores/router';
import { Footer } from '@cmp/footer.comp';
import { Header } from '@cmp/header.comp';
import { Home } from '@pag/home.page';
import { render } from 'preact';
import './app.css';

export function App() {
    return (
        <>
            <Header />
            <main class='container mx-auto px-4 py-2'>
                <Home />
            </main>
            <Footer />
        </>
    );
}

render(<App />, document.getElementById('app'));
