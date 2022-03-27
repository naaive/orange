import * as React from 'react';
import * as ReactDOM from 'react-dom';
import './index.css'
import App from "./App";
import {mergeStyles, ThemeProvider,initializeIcons} from '@fluentui/react';

initializeIcons()
// Inject some global styles
mergeStyles({
    ':global(body,html,#root)': {
        margin: 0,
        padding: 0,
        height: '100vh',
    },
});

ReactDOM.render(<>
    <ThemeProvider>
    <App/>
    </ThemeProvider>
</>, document.getElementById('root'));


