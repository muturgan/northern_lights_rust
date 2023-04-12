(() => {
    const promoInput = document.body.querySelector('#promo-input-promo');
    const phoneInput = document.body.querySelector('#promo-input-phone');
    const output = document.body.querySelector('#promo-output');
    const checkButton = document.body.querySelector('#check-button-check');
    const activateButton = document.body.querySelector('#check-button-activate');
    if (promoInput === null || phoneInput === null || output === null || checkButton === null || activateButton === null) {
        throw new Error('incomplete html');
    }
    promoInput.addEventListener('input', () => {
        promoInput.value = promoInput.value.toUpperCase();
    });
    const CHECK_URL = 'api/check';
    const ACTIVATE_URL = 'api/activate';
    let pass = '';
    const phoneRe = /^(\+7)\d{10}$/;
    const cleanupRe = /\s+|-|\(|\)/g;
    const promoRe = /^[а-я]{4,8}-\d{3}$/;
    const postData = {
        promocode: '',
        phone: '',
    };
    const enterPass = () => {
        pass = prompt('Введите пароль') || '';
    };
    const checkPromo = (promo) => {
        const trimed = promo.trim().toLowerCase();
        if (trimed.length === 0) {
            postData.promocode = '';
            return 'Промокод обязателен для ввода';
        }
        if (trimed.length < 8 || trimed.length > 12) {
            postData.promocode = '';
            return 'Вы ввели некорректный промокод';
        }
        const valid = promoRe.test(trimed);
        postData.promocode = valid === true ? trimed : '';
        return valid === true ? null : 'Вы ввели некорректный промокод';
    };
    const checkPhone = (phone) => {
        const isString = typeof phone === 'string' && phone.length > 0;
        if (isString !== true) {
            postData.phone = '';
            return 'Tелефон обязателен для ввода';
        }
        const trimed = phone.replace(cleanupRe, '');
        const transformed = trimed.startsWith('8') ? '+7' + trimed.substr(1) : trimed;
        const valid = phoneRe.test(transformed);
        postData.phone = valid === true ? transformed : '';
        return valid === true ? null : 'Вы указали некорректный номер телефона';
    };
    const blockWorkspace = () => {
        checkButton.disabled = true;
        activateButton.disabled = true;
        promoInput.disabled = true;
        phoneInput.disabled = true;
        checkButton.classList.add('disabled');
        activateButton.classList.add('disabled');
    };
    const unblockWorkspace = () => {
        promoInput.disabled = false;
        phoneInput.disabled = false;
        checkButton.disabled = false;
        activateButton.disabled = false;
        checkButton.classList.remove('disabled');
        activateButton.classList.remove('disabled');
    };
    const handleApiResponse = (res, btnElem) => {
        pass = '';
        setTimeout(() => {
            output.innerText = res.result;
            switch (res.status) {
                case 0 /* EScenarioStatus.SCENARIO_SUCCESS */:
                    output.classList.add('info');
                    output.classList.remove('warning');
                    output.classList.remove('error');
                    break;
                case 1 /* EScenarioStatus.UNAUTHORIZED */:
                    output.classList.remove('info');
                    output.classList.add('warning');
                    output.classList.remove('error');
                    enterPass();
                    break;
                case 2 /* EScenarioStatus.SCENARIO_FAIL */:
                    output.classList.remove('info');
                    output.classList.add('warning');
                    output.classList.remove('error');
                    break;
                case 3 /* EScenarioStatus.SYSTEM_ERROR */:
                    output.classList.remove('info');
                    output.classList.remove('warning');
                    output.classList.add('error');
                    break;
                default:
                    output.classList.remove('info');
                    output.classList.remove('warning');
                    output.classList.remove('error');
            }
            postData.promocode = '';
            postData.phone = '';
            unblockWorkspace();
            btnElem.classList.remove('fetching');
        }, 1000);
    };
    const handleApiError = (err, btnElem) => {
        console.info('api error:');
        console.info(err);
        setTimeout(() => {
            output.innerText = 'Извините, произошла ошибка при обращиении к серверу';
            output.classList.remove('info');
            output.classList.remove('warning');
            output.classList.add('error');
            postData.promocode = '';
            postData.phone = '';
            unblockWorkspace();
            btnElem.classList.remove('fetching');
        }, 1000);
    };
    const handleNot200 = async (status, statusText, res) => {
        console.info('Api call fail...');
        console.info('status:', status);
        console.info('statusText:', statusText);
        const body = await res;
        throw body;
    };
    const fetchData = (url, btnElem) => {
        blockWorkspace();
        btnElem.classList.add('fetching');
        const reqOptions = {
            method: 'post',
            body: JSON.stringify(postData),
            headers: {
                'Content-Type': 'application/json',
            },
        };
        if (pass) {
            reqOptions.headers.authorization = pass;
        }
        fetch(url, reqOptions)
            .then((raw) => {
            return raw.ok === true
                ? raw.json()
                : handleNot200(raw.status, raw.statusText, raw.text());
        })
            .then((res) => handleApiResponse(res, btnElem))
            .catch((err) => handleApiError(err, btnElem));
    };
    checkButton.addEventListener('click', () => {
        const validationResults = [
            checkPromo(promoInput.value),
            checkPhone(phoneInput.value),
        ];
        const errors = validationResults.filter((r) => r !== null);
        if (errors.length > 0) {
            const errorMessage = errors.join('; ');
            output.innerText = errorMessage;
            output.classList.remove('info');
            output.classList.add('warning');
            output.classList.remove('error');
        }
        else {
            fetchData(CHECK_URL, checkButton);
        }
    });
    activateButton.addEventListener('click', () => {
        const validationResults = [
            checkPromo(promoInput.value),
            checkPhone(phoneInput.value),
        ];
        const errors = validationResults.filter((r) => r !== null);
        if (errors.length > 0) {
            const errorMessage = errors.join('; ');
            output.innerText = errorMessage;
            output.classList.remove('info');
            output.classList.add('warning');
            output.classList.remove('error');
        }
        else {
            fetchData(ACTIVATE_URL, activateButton);
        }
    });
    window.addEventListener('load', enterPass, { once: true });
})();

