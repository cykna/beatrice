function rand(){
    return 12;
}

function main(){
    const a = 5;
    let b;
    if(a) b = a*a;
    else {
        const q = rand() ? 24 : 12
        if(q) b = q;
        else b = 50;
    }
}

